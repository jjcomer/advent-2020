(ns jjcomer.day1
  [:require [clojure.string :as str]
   [jjcomer.util :as util]]
  (:require [clojure.java.io :as io]))

(defn parse-input [lines]
  (->> lines
       (map str/trim)
       (map #(Integer. %))))

(defn solve-part-1 []
  (let [input (parse-input (util/get-input 1))
        result (loop [acc #{} nums input]
                 (if-let [n (first nums)]
                   (let [comp (- 2020 n)]
                     (if (acc comp)
                       (* n comp)
                       (recur (conj acc n) (rest nums))))
                   -1))]
    result))

(defn solve-part-2 []
  (let [input (set (parse-input (util/get-input 1)))
        result (for [x input
                     y input
                     :let [comp (- 2020 x y)]
                     :when (and (not= y comp) (not= x comp) (not= x y))]
                 (when (input comp)
                   (* x y comp)))]
    (first (keep identity result))))
