(ns y2016.d6
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/6

;; Solution Logic

(defn find-most-frequent [letters]
  (let [f (sort-by identity (fn [a b]
                              (compare (val b) (val a)))
                   (frequencies letters))]
    (key (first f))))

(defn find-least-frequent [letters]
  (let [f (sort-by identity (fn [a b]
                              (compare (val a) (val b)))
                   (frequencies letters))]
    (key (first f))))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (-> input
      str/trim
      str/split-lines))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (->> (range (count (first input)))
       (map #(map (fn [l] (nth l %)) input))
       (map find-most-frequent)
       (apply str)))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (->> (range (count (first input)))
       (map #(map (fn [l] (nth l %)) input))
       (map find-least-frequent)
       (apply str)))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
