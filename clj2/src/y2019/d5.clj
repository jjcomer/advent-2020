(ns y2019.d5
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2019/day/5

;; Generator Logic

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

;; Solution Logic

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

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (parse-intcode input))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (let [[new-program input output] (execute-intcode input [1])]
    output))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (let [[new-program input output] (execute-intcode input [5])]
    output))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
