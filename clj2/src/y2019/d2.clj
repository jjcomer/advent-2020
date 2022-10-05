(ns y2019.d2
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2019/day/2

;; Generator Logic

(defn parse-intcode [raw-program]
  (reduce (fn [acc [location code]]
            (assoc acc location (parse-long code)))
          {}
          (map-indexed vector (str/split raw-program #","))))

;; Solution Logic

(defn indirect-lookup [program location]
  (get program (get program location)))

(defn process-intcode [program pointer]
  (case (get program pointer)
    1 (let [arg1 (indirect-lookup program (+ 1 pointer))
            arg2 (indirect-lookup program (+ 2 pointer))
            arg3 (get program (+ 3 pointer))
            result (+ arg1 arg2)]
        [:continue (+ 4 pointer) (assoc program arg3 result)])
    2 (let [arg1 (indirect-lookup program (+ 1 pointer))
            arg2 (indirect-lookup program (+ 2 pointer))
            arg3 (get program (+ 3 pointer))
            result (* arg1 arg2)]
        [:continue (+ 4 pointer) (assoc program arg3 result)])
    99 [:halt pointer program]))

(defn execute-intcode [program]
  (loop [program program pointer 0]
    (let [[result new-pointer new-program] (process-intcode program pointer)]
      (case result
        :continue (recur new-program new-pointer)
        :halt new-program))))

(defn noun-verb-run [program noun verb]
  (let [changed-program (-> program
                            (assoc 1 noun)
                            (assoc 2 verb))
        final-state (execute-intcode changed-program)]
    (get final-state 0)))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (parse-intcode input))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (noun-verb-run input 12 2))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (let [params (for [x (range 0 100) y (range 0 100)] [x y])
        [noun verb] (->> params
                         (filter (fn [[noun verb]]
                                   (= 19690720 (noun-verb-run input noun verb))))
                         first)]
    (+ verb (* noun 100))))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
