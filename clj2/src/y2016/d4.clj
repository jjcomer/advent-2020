(ns y2016.d4
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/4

(def room-regex #"(.*)-(\d+)\[(.*)\]")

(defn parse-room [r]
  (let [[_ letters code checksum] (re-find room-regex r)]
    {:letters letters
     :code (parse-long code)
     :checksum checksum}))

;; Solution Logic

(defn generate-checksum [letters]
  (->> letters
       seq
       (filter #(not= % \-))
       frequencies
       (sort-by identity (fn [a b]
                           (if (= (val a) (val b))
                             (compare (key b) (key a))
                             (compare (val a) (val b)))))
       (map key)
       reverse
       (take 5)
       (apply str)))

(defn rotate [c x]
  (let [base (- (int c) 96)]
    (char (+ 96 (mod (+ base x) 26)))))

(defn rotate-letters [letters code]
  (->> letters
       (map #(if (not= \- %)
               (rotate % code)
               %))
       (apply str)))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (->> input
       str/trim
       str/split-lines
       (map parse-room)))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (->> input
       (filter #(= (:checksum %) (generate-checksum (:letters %))))
       (map :code)
       (reduce +)))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (->> input
       (filter #(= (:checksum %) (generate-checksum (:letters %))))
       (map #(assoc % :name (rotate-letters (:letters %) (:code %))))
       (filter #(str/includes? (:name %) "north"))
       first
       :code))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
