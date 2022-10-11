(ns y2019.intcode
  (:require [clojure.string :as str]
            [clojure.core.async :as a]))

(def log (a/chan 100000))


(def start-logger
  (memoize
   (fn []
     (a/go-loop []
       (let [m (a/<! log)]
         #_(println m))
       (recur)))))

(defn parse-modes [code]
  (let [full (->> code
                  (format "%05d")
                  (map #(Character/getNumericValue %))
                  (take 3)
                  (map #(case %
                          1 :immediate
                          0 :position
                          :unknown)))]

    {:arg1 (nth full 2)
     :arg2 (nth full 1)
     :arg3 (nth full 0)}))

(defn parse-code [code]
  {:value code
   :op-code (mod code 100)
   :modes (parse-modes code)})

(defn parse-intcode [raw-program]
  (reduce (fn [acc [location code]]
            (assoc acc location (-> code
                                    parse-long
                                    parse-code)))
          {}
          (map-indexed vector (str/split (str/trim raw-program) #","))))

(defn lookup-value [program mode location]
  (case mode
    :immediate (:value (get program location))
    :position (:value (get program (:value (get program location))))))

(defn process-intcode [program input output pointer]
  (let [{:keys [op-code modes] :as raw} (get program pointer)]
    ;;(println raw)
    (case op-code
      ;; Add
      1 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
              arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
              arg3 (lookup-value program :immediate (+ 3 pointer))
              result (+ arg1 arg2)]
          [:continue
           (+ 4 pointer)
           (assoc program arg3 (parse-code result))
           input
           output])
      ;; Multiply
      2 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
              arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
              arg3 (lookup-value program :immediate (+ 3 pointer))
              result (* arg1 arg2)]
          [:continue
           (+ 4 pointer)
           (assoc program arg3 (parse-code result))
           input
           output])
      ;; Input
      3 (let [arg1 (lookup-value program :immediate (+ 1 pointer))]
          [:continue
           (+ 2 pointer)
           (assoc program arg1 (parse-code (first input)))
           (rest input)
           output])
      ;; Output
      4 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))]
          [:continue
           (+ 2 pointer)
           program
           input
           (conj output arg1)])
      ;; Jump-if-true
      5 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
              arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))]
          [:continue
           (if-not (zero? arg1)
             arg2
             (+ 3 pointer))
           program
           input
           output])
      ;; Jump-if-false
      6 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
              arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))]
          [:continue
           (if (zero? arg1)
             arg2
             (+ 3 pointer))
           program
           input
           output])
      ;; Less than
      7 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
              arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
              arg3 (lookup-value program :immediate (+ 3 pointer))]
          [:continue
           (+ 4 pointer)
           (assoc program arg3 (parse-code (if (< arg1 arg2) 1 0)))
           input
           output])
      ;; Equals
      8 (let [arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
              arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
              arg3 (lookup-value program :immediate (+ 3 pointer))]
          [:continue
           (+ 4 pointer)
           (assoc program arg3 (parse-code (if (= arg1 arg2) 1 0)))
           input
           output])
      ;; Halt
      99 [:halt
          pointer
          program
          input
          output])))

(defn execute-intcode [program input]
  (loop [program program pointer 0 input input output []]
    (let [[result new-pointer new-program input output] (process-intcode program
                                                                         input
                                                                         output
                                                                         pointer)]
      (case result
        :continue (recur new-program new-pointer input output)
        :halt [new-program input output]))))

(defn execute-async-intcode
  ([program]
   (execute-async-intcode program (a/chan 10) (a/chan 10)))
  ([program input<- output-> n]
   (start-logger)
   (a/go-loop
       [program program pointer 0]
     #_(a/>! log (format "Loop %s %d" n pointer))
     #_(a/<! (a/timeout 100))
     (let [[result new-pointer new-program]
           (let [{:keys [op-code modes] :as raw} (get program pointer)]
             ;;(println raw)
             (case op-code
               ;; Add
               1 (let [_ (a/>! log (format "ADD %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
                       arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
                       arg3 (lookup-value program :immediate (+ 3 pointer))
                       result (+ arg1 arg2)]
                   [:continue
                    (+ 4 pointer)
                    (assoc program arg3 (parse-code result))])
               ;; Multiply
               2 (let [_ (a/>! log (format "MULTIPLY %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
                       arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
                       arg3 (lookup-value program :immediate (+ 3 pointer))
                       result (* arg1 arg2)]
                   [:continue
                    (+ 4 pointer)
                    (assoc program arg3 (parse-code result))])
               ;; Input
               3 (let [_ (a/>! log (format "INPUT %s %d" n pointer))
                       arg1 (lookup-value program :immediate (+ 1 pointer))]
                   [:continue
                    (+ 2 pointer)
                    (assoc program arg1 (parse-code (a/<! input<-)))])
               ;; Output
               4 (let [_ (a/>! log (format "OUTPUT %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))]
                   (when-not(a/>! output-> arg1)
                     (a/>! log (format "OUTPUT CLOSED %s" n)))
                   #_(a/>! log arg1)
                   [:continue
                    (+ 2 pointer)
                    program])
               ;; Jump-if-true
               5 (let [_ (a/>! log (format "JUMP TRUE %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
                       arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))]
                   [:continue
                    (if-not (zero? arg1)
                      arg2
                      (+ 3 pointer))
                    program])
               ;; Jump-if-false
               6 (let [_ (a/>! log (format "JUMP FALSE %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
                       arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))]
                   [:continue
                    (if (zero? arg1)
                      arg2
                      (+ 3 pointer))
                    program])
               ;; Less than
               7 (let [_ (a/>! log (format "LESS THAN %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
                       arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
                       arg3 (lookup-value program :immediate (+ 3 pointer))]
                   [:continue
                    (+ 4 pointer)
                    (assoc program arg3 (parse-code (if (< arg1 arg2) 1 0)))])
               ;; Equals
               8 (let [_ (a/>! log (format "EQUALS %s %d" n pointer))
                       arg1 (lookup-value program (:arg1 modes) (+ 1 pointer))
                       arg2 (lookup-value program (:arg2 modes) (+ 2 pointer))
                       arg3 (lookup-value program :immediate (+ 3 pointer))]
                   [:continue
                    (+ 4 pointer)
                    (assoc program arg3 (parse-code (if (= arg1 arg2) 1 0)))])
               ;; Halt
               99 (do
                    (a/>! log (format "HALT %s %d" n pointer))
                    (a/close! input<-)
                    (a/close! output->)
                    [:halt
                     pointer
                     program])))]
       (case result
         :continue (recur new-program new-pointer)
         :halt (do
                 (a/>! log (format "Loop %s HALT" n))
                 [new-program output->]))))))
