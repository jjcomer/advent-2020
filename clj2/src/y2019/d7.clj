(ns y2019.d7
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.math.combinatorics :as c]
            [y2019.intcode :as ic]
            [clojure.core.async :as a]))

;; PROBLEM LINK https://adventofcode.com/2019/day/7

;; Solution Logic

(defn execute-sequence [program sequence]
  (reduce (fn [acc s]
            (last (last (ic/execute-intcode program [s acc]))))
          0
          sequence))

(defn make-chan [s input]
  (let [output (a/chan 10)]
    (a/>!! input s)
    output))

(defn execute-loop-sequence [program sequence]
  #_(a/<!! (a/timeout 1000))
  ;;(println (pr-str sequence))
  (let [[a b c d e] sequence
        a-in (a/chan 10)
        a-mult (a/mult a-in)
        real-a-in (a/chan 10)
        final-output (a/chan (a/sliding-buffer 1))
        _ (a/tap a-mult real-a-in)
        _ (a/tap a-mult final-output)
        a-out (make-chan a a-in)
        b-out (make-chan b a-out)
        c-out (make-chan c b-out)
        d-out (make-chan d c-out)
        e-out (make-chan e d-out)
        start (partial ic/execute-async-intcode program)]
    (a/>!! a-in 0)
    (start real-a-in a-out 0) ;;a
    (start a-out b-out 1) ;;b
    (start b-out c-out 2) ;;c
    (start c-out d-out 3) ;;d
    (let [final (start d-out a-in 4)] ;;e
      (a/<!! final)
      (a/<!! final-output))))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (ic/parse-intcode input))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (let [t (map (partial execute-sequence input))]
    (transduce t (completing max) 0 (c/permutations [0 1 2 3 4]))))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  #_(execute-loop-sequence input [5 6 9 7 8])
  (let [t (map (partial execute-loop-sequence input))]
    (transduce t (completing max) 0 (c/permutations [5 6 7 8 9]))))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))

(def sample1
  "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")

(deftest sample1-test
  (let [program (generator sample1)]
    (t/is (= 139629729 (execute-loop-sequence program [9 8 7 6 5])))))
