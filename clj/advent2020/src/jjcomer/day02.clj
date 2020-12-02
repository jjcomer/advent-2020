(ns jjcomer.day02
  (:require [clojure.string :as str]
            [jjcomer.util :as util]))

(defn parse-input [lines]
  (map (fn [line]
         (let [[limits lookup password] (str/split line #" ")
               [lower upper] (map #(Integer. %) (str/split limits #"-"))
               lookup (first lookup)]
           {:lower lower
            :upper upper
            :lookup lookup
            :password password}))
       lines))

(defn part-1-test [{:keys [password lower upper lookup]}]
  (let [num (count (filter #(= lookup %) password))]
    (and (>= num lower)
         (<= num upper))))

(defn solve-part-1 []
  (let [input (parse-input (util/get-input 2))]
    (count (filter part-1-test input))))

(defn part-2-test [{:keys [password lower upper lookup]}]
  (let [first-c (= lookup (nth password (dec lower)))
        second-c (= lookup (nth password (dec upper)))]
    (or (and first-c (not second-c))
        (and (not first-c) second-c))))

(defn solve-part-2 []
  (let [input (parse-input (util/get-input 2))]
    (count (filter part-2-test input))))
