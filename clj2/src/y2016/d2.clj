(ns y2016.d2
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/2

;; Solution Logic

(defn move-number [n instruction]
  (case instruction
    \U
    (case n
      (1 2 3) n
      (- n 3))
    \D
    (case n
      (7 8 9) n
      (+ n 3))
    \L
    (case n
      (1 4 7) n
      (dec n))
    \R
    (case n
      (3 6 9) n
      (inc n))))

(defn move-number-2 [n instruction]
  (case instruction
    \U
    (case n
      (5 2 1 4 9) n
      (13) 11
      (3) 1
      (- n 4))
    \D
    (case n
      (5 10 13 12 9) n
      (1) 3
      (11) 13
      (+ n 4))
    \L
    (case n
      (1 2 5 10 13) n
      (dec n))
    \R
    (case n
      (1 4 9 12 13) n
      (inc n))))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (->> input
       str/trim
       str/split-lines
       (map seq)))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (->> input
       (reductions (fn [n instructions]
                     (reduce move-number n instructions))
                   5)
       rest))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (->> input
       (reductions (fn [n instructions]
                     (reduce move-number-2 n instructions))
                   5)
       rest))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
