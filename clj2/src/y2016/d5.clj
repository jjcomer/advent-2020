(ns y2016.d5
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str])
  (:import (java.security MessageDigest)
           (java.math BigInteger)))

;; PROBLEM LINK https://adventofcode.com/2016/day/5

;; Solution Logic

(defn md5 [^String s]
  (let [algorithm (MessageDigest/getInstance "MD5")
        raw (.digest algorithm (.getBytes s))]
    (format "%032x" (BigInteger. 1 raw))))

(defn find-password-char [base i]
  (let [hash (md5 (str base i))]
    (when (str/starts-with? hash "00000")
      (println i hash)
      (nth hash 5))))

(defn safe-parse-long [x]
  (let [d (Character/digit x 10)]
    (when (>= d 0)
      d)))

(defn find-password-char-2 [base i]
  (let [hash (md5 (str base i))
        index (safe-parse-long (nth hash 5))]
    (when (and (str/starts-with? hash "00000")
               index)
      (println i hash)
      [index (nth hash 6)])))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (str/trim input))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (->> (range)
       (keep #(find-password-char input %))
       (take 8)
       (apply str)))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (let [password (reduce (fn [password i]
                           (if-let [[index val] (find-password-char-2 input i)]
                             (let [password (if (and (not (contains? password index))
                                                     (<= 0 index 7))
                                              (assoc password index val)
                                              password)]
                               (if (= 8 (count password))
                                 (reduced password)
                                 password))
                             password))
                         (sorted-map)
                         (range))]
    (println password)
    (apply str (vals password))))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
