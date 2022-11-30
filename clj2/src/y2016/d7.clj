(ns y2016.d7
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/7

(def pair-regex #"(\w{1})(\w{1})\2\1")

(defn parse-line [l]
  (let [blocks (str/split l #"[\[\]]")]
    (map (fn [block hyper]
           {:hyper hyper
            :block block})
         blocks
         (cycle [false true]))))

;; Solution Logic
(defn tls-test [block]
  (when-let [[_ a b] (re-find pair-regex block)]
    (not= a b)))

(defn support-tls? [blocks]
  (and (every? #(not (tls-test (:block %))) (filter :hyper blocks))
       (some #(tls-test (:block %)) (remove :hyper blocks))))

(defn get-bab-patterns [block]
  (->> block
       (partition 3 1)
       (filter (fn [[a b c]]
                 (and (= a c) (not= a b))))
       (map (fn [[a b _]] (re-pattern (str b a b))))))

(defn support-ssl? [blocks]
  (let [patterns (->> blocks
                      (remove :hyper)
                      (map :block)
                      (mapcat get-bab-patterns))]
    (when (seq patterns)
      (some (fn [block]
              (some #(re-find % block) patterns))
            (map :block (filter :hyper blocks))))))
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
  (->> input
       (filter support-tls?)
       count))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (->> input
       (filter support-ssl?)
       count))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
