(ns jjcomer.util
  (:require [clojure.java.io :as io]))

(defn get-input [day & {:keys [lines] :or {lines true}}]
  (let [reader (io/reader (io/resource (str "2020/day" day ".txt")))]
    (if lines
      (line-seq reader)
      (slurp reader))))
