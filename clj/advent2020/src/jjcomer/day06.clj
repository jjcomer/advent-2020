(ns jjcomer.day06
  (:require [jjcomer.util :as util]
            [clojure.string :as str]
            [clojure.set :as set]))

(defn solve-part-1 []
  (transduce (comp (map #(set (str/replace % "\n" "")))
                   (map count))
             +
             (str/split (util/get-input 6 :lines false) #"\n\n")))

(defn solve-part-2 []
  (transduce (comp (map #(reduce set/intersection (map set (str/split % #"\n"))))
                   (map count))
             +
             (str/split (util/get-input 6 :lines false) #"\n\n")))
