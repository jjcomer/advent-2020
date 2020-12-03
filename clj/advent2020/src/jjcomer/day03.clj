(ns jjcomer.day03
  (:require [jjcomer.util :as util]))

(defn check-for-trees [landscape x y]
  (let [height (count landscape)
        width (count (first landscape))]
    (count (filter (fn [i] (and (< (* y i) height)
                                (= \# (nth (nth landscape (* y i)) (mod (* x i) width)))))
                   (range height)))))

(defn solve-part1 []
  (let [input (util/get-input 3)]
    (check-for-trees input 3 1)))

(defn solve-part2 []
  (let [input (util/get-input 3)]
    (* (check-for-trees input 1 1)
       (check-for-trees input 3 1)
       (check-for-trees input 5 1)
       (check-for-trees input 7 1)
       (check-for-trees input 1 2))))
