(ns y2020.d20
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.set :as set]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2020/day/20

;; Solution Logic

(defn neighbour? [tile-a tile-b]
  (seq (set/intersection (set (vals (:edges tile-a)))
                         (set (vals (:edges tile-b))))))

(defn find-neighbours [grid]
  (map (fn [tile-a]
         (assoc tile-a
                :neighbours
                (map :id (filter (fn [tile-b] (neighbour? tile-a tile-b))
                                 (filter #(not= tile-a %) grid)))))
       grid))

(def sample-tile
  "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###")

;; Generator logic
(defn rotate [tile]
  (mapv #(mapv (fn [l] (nth l %)) tile)
        (range (count tile))))

(defn flip [tile]
  (mapv rseq tile))

(defn rotations [tile]
  (let [north tile
        f-north (flip north)
        east (rotate north)
        f-east (flip east)
        south (rotate east)
        f-south (flip south)
        west (rotate south)
        f-west (flip west)]
    (into #{} [north
               f-north
               east
               f-east
               south
               f-south
               west
               f-west])))

(defn all-edges [tile]
  (let [north (first tile)
        south (last tile)
        west (mapv first tile)
        east (mapv last tile)]
    {:north north
     :f-north (rseq north)
     :south south
     :f-south (rseq south)
     :west west
     :f-west (rseq west)
     :east east
     :f-east (rseq east)}))

(defn parse-tile [raw-tile]
  (let [lines (str/split-lines raw-tile)
        number (->> (first lines)
                    (re-matches #"Tile (\d*):")
                    second
                    parse-long)
        tile (mapv #(into [] %) (rest lines))]
    {:id number
     :base-tile tile
     :edges (all-edges tile)
     :rotations (rotations tile)}))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (mapv parse-tile (str/split input #"\n\n")))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (let [the-grid (find-neighbours input)]
    (println (map #(count (:neighbours %)) the-grid))
    (apply * (map :id (filter #(= 2 (count (:neighbours %))) the-grid)))))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input])

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
