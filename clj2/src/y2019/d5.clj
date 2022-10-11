(ns y2019.d5
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]
            [y2019.intcode :as ic]))

;; PROBLEM LINK https://adventofcode.com/2019/day/5

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (ic/parse-intcode input))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (let [[new-program input output] (ic/execute-intcode input [1])]
    output))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (let [[new-program input output] (ic/execute-intcode input [5])]
    output))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
