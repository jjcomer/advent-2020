(ns y2017.d10
  (:require [clojure.test :as t :refer [deftest]]))

;; PROBLEM LINK https://adventofcode.com/2017/day/10

;; Solution Logic

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input])

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input])

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input])

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
