(ns y2016.d1
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/1

;; Solution Logic

(defn turn [direction turn-direction]
  (case turn-direction
    \L (case direction
         :N :W :E :N :S :E :W :S)
    \R (case direction
         :N :E :E :S :S :W :W :N)))

(defn move [[x y] direction distance]
  (case direction
    :N [x (+ y distance)]
    :E [(+ x distance) y]
    :S [x (- y distance)]
    :W [(- x distance) y]))

(defn move2 [[x y] direction distance]
  (case direction
    :N (map (fn [i] [x (+ y i)]) (range 1 (inc distance)))
    :E (map (fn [i] [(+ x i) y]) (range 1 (inc distance)))
    :S (map (fn [i] [x (- y i)]) (range 1 (inc distance)))
    :W (map (fn [i] [(- x i) y]) (range 1 (inc distance)))))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (-> input
      (str/trim)
      (str/split #", ")
      (#(map (fn [x] [(first x)
                      (parse-long (apply str (rest x)))]) %))))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (loop [location [0 0] direction :N instructions input]
    (if-let [[turn-direction distance] (first instructions)]
      (let [new-direction (turn direction turn-direction)
            new-location (move location new-direction distance)]
        (recur new-location new-direction (rest instructions)))
      (+ (abs (first location)) (abs (second location))))))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (loop [location [0 0] direction :N instructions input visited #{}]
    (if-let [[turn-direction distance] (first instructions)]
      (let [new-direction (turn direction turn-direction)
            new-locations (move2 location new-direction distance)]
        (if-let [found-it (first (filter visited (rest new-locations)))]
          (+ (abs (first found-it)) (abs (second found-it)))
          (recur (last new-locations) new-direction (rest instructions) (into visited new-locations))))
      :error)))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))

(deftest sample-part2
  (let [input (generator "R8, R4, R4, R8")
        result (solve-part-2 input)]
    (t/is (= result 4))))