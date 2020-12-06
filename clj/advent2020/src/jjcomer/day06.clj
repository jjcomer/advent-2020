(ns jjcomer.day06
  (:require [jjcomer.util :as util]
            [clojure.string :as str]
            [clojure.set :as set]))

(defn solve-part-1 []
  (let [input (map #(set (str/replace % "\n" ""))
                   (str/split (util/get-input 6 :lines false) #"\n\n"))]
    (reduce + (map count input))))

(defn solve-part-2 []
  (let [input (map #(reduce set/intersection (map set (str/split % #"\n")))
                   (str/split (util/get-input 6 :lines false) #"\n\n"))]
    (reduce + (map count input))))
