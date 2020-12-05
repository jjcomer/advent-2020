(ns jjcomer.day05
  (:require [jjcomer.util :as util]))

(defn walk-tree [directions up max]
  (loop [min 0 max max directions directions]
    (if-let [next-direction (first directions)]
      (let [offset (quot (+ min max) 2)]
        (if (= up next-direction)
          (recur min (dec offset) (rest directions))
          (recur (inc offset) max (rest directions))))
      min)))

(defn find-seat [directions]
  (let [row (walk-tree (subs directions 0 7) \F 127)
        column (walk-tree (subs directions 7 (count directions)) \L 7)]
    (+ column (* row 8))))

(defn solve-part-1 []
  (let [directions (util/get-input 5)]
    (reduce max (map find-seat directions))))

(defn solve-part-2 []
  (let [seats (set (map find-seat (util/get-input 5)))]
    (loop [tickets seats]
      (if-let [to-check (first tickets)]
        (if (and (seats (+ 2 to-check))
                 (not (seats (inc to-check))))
          (inc to-check)
          (recur (rest tickets)))))))
