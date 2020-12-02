(ns jjcomer.util
  (:require [clojure.java.io :as io]))

(defn get-input [day]
  (line-seq (io/reader (io/resource (str "2020/day" day ".txt")))))
