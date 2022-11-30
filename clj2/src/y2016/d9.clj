(ns y2016.d9
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/9

;; Solution Logic

(def expand-regex #"(\d+)x(\d+)\)")

(defn compute-length [data]
  (loop [data data length 0]
    (if-let [start-index (str/index-of data "(")]
      (let [length (+ length start-index)
            new-data (subs data (inc start-index))
            [expand-match e-length e-times] (re-find expand-regex new-data)]
        (recur (subs new-data (+ (count expand-match) (parse-long e-length)))
               (+ length (* (parse-long e-length)
                            (parse-long e-times)))))
      (+ length (count data)))))

(defn compute-length2 [data]
  (loop [data data length 0]
    (if-let [start-index (str/index-of data "(")]
      (let [length (+ length start-index)
            new-data (subs data (inc start-index))
            [expand-match e-length e-times] (re-find expand-regex new-data)
            e-length (parse-long e-length)
            e-times (parse-long e-times)
            repeated-data (subs new-data (count expand-match) (+ (count expand-match) e-length))
            added-length (compute-length2 repeated-data)]
        (recur (subs new-data (+ (count expand-match) e-length))
               (+ length (* added-length e-times))))
      (+ length (count data)))))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (str/trim input))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (compute-length input))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (compute-length2 input))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))

(deftest part1-test
  (let [tests [["ADVENT" 6]
               ["A(1x5)BC" 7]
               ["(3x3)XYZ" 9]
               ["A(2x2)BCD(2x2)EFG" 11]
               ["(6x1)(1x3)A" 6]
               ["X(8x2)(3x3)ABCY" 18]]]
    (doseq [[test expected] tests]
      (t/is (= (compute-length test) expected) test))))

(deftest part2-test
  (let [tests [["(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN" 445]]]
    (doseq [[test expected] tests]
      (t/is (= (compute-length2 test) expected) test))))