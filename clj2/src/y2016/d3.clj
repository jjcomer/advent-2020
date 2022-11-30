(ns y2016.d3
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/3

;; Generator Logic

;; Solution Logic

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (->> input
       str/trim
       str/split-lines
       (map #(map parse-long (str/split (str/trim %) #" +")))))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (->> input
       (map sort)
       (filter #(> (+ (first %) (second %)) (nth % 2)))
       count))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (->> input
       (partition 3)
       (mapcat #(map (fn [i] (map (fn [x] (nth x i)) %)) [0 1 2]))
       (map sort)
       (filter #(> (+ (first %) (second %)) (nth % 2)))
       count))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
