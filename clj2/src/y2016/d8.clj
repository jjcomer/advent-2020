(ns y2016.d8
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/8

(def rect-regex #"rect (\d+)x(\d+)")
(def rotate-regex #"rotate (row|column) [yx]=(\d+) by (\d+)")

(defn parse-line [line]
  (if-let [[_ x y] (re-find rect-regex line)]
    {:type :rect
     :x (parse-long x)
     :y (parse-long y)}
    (if-let [[_ target pos amount] (re-find rotate-regex line)]
      {:type (keyword target)
       :pos (parse-long pos)
       :amount (parse-long amount)}
      (throw (ex-info "Error" {:line line})))))

;; Solution Logic

(defn rotate [v n]
  (cond
    (zero? n) v
    (pos? n) (vec (concat (nthrest v n) (take n v)))
    (neg? n) (rotate v (+ (count v) n))))

(defn draw-rect [grid x y]
  ;;(println "rect" x y)
  (let [points (for [x (range x) y (range y)] [x y])]
    (reduce (fn [grid key] (assoc grid key true)) grid points)))

(defn rotate-row [grid row amount]
  (let [points (for [x (range 50)] [x row])
        data (map #(get grid % false) points)
        rotated-data (rotate data (- amount))]
    ;;(println :row row amount)
    (reduce (fn [grid [x _ :as key]]
              (assoc grid key (nth rotated-data x)))
            grid
            points)))

(defn rotate-column [grid column amount]
  (let [points (for [y (range 6)] [column y])
        data (map #(get grid % false) points)
        rotated-data (rotate data (- amount))]
    ;;(println :column column amount)
    (reduce (fn [grid [_ y :as key]]
              (assoc grid key (nth rotated-data y)))
            grid
            points)))

(defn execute-command [grid command]
  (case (:type command)
    :rect (draw-rect grid (:x command) (:y command))
    :row (rotate-row grid (:pos command) (:amount command))
    :column (rotate-column grid (:pos command) (:amount command))))

(defn print-grid [grid]
  (loop [y 0]
    (when (not= 7 y)
      (loop [x 0]
        (when (not= 51 x)
          (print (if (get grid [x y] false)
                   "#" " "))
          (recur (inc x))))
      (println)
      (recur (inc y))))
  (println))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (->> input
       str/trim
       str/split-lines
       (map parse-line)))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (loop [commands input grid {}]
    ;;(print-grid grid)
    (if-let [command (first commands)]
      (recur (rest commands)
             (execute-command grid command))
      (->> grid
           vals
           (filter identity)
           count))))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (loop [commands input grid {}]
    (if-let [command (first commands)]
      (recur (rest commands)
             (execute-command grid command))
      (print-grid grid))))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
