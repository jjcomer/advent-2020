(ns y2020.d20
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.set :as set]
            [clojure.string :as str]
            [clojure.pprint :as pp]))

;; PROBLEM LINK https://adventofcode.com/2020/day/20

;; Solution Logic

(defn neighbour? [tile-a tile-b]
  (set/intersection (set (vals (:edges tile-a)))
                    (set (vals (:edges tile-b)))))

(defn find-neighbours [grid]
  (map (fn [tile-a]
         (let [neighbours (map (fn [tile-b]
                                 (let [edges (neighbour? tile-a tile-b)]
                                   {:id (:id tile-b)
                                    :edge (reduce-kv (fn [acc orientation edge]
                                                       (if (edges edge)
                                                         (assoc acc orientation edge)
                                                         acc))
                                                     {}
                                                     (:edges tile-a))}))
                               (filter (fn [tile-b] (seq (neighbour? tile-a tile-b)))
                                       (filter #(not= tile-a %) grid)))]
           (assoc tile-a :neighbours neighbours)))
       grid))

(defn get-east [tile]
  (map last tile))

(defn get-west [tile]
  (map first tile))

(defn get-south [tile]
  (last tile))

(defn get-north [tile]
  (first tile))

(defn n-s-match [test-tile n-tile s-tile]
  (let [fixed-north (get-south (:base-tile n-tile))
        possible-orientations (filter #(= fixed-north (get-north %)) (:rotatations test-tile))]))

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

(defn h-flip [tile]
  (mapv rseq tile))

(defn v-flip [tile]
  (rseq tile))

(defn rotations [tile]
  (let [north tile
        f-north (h-flip north)
        v-north (v-flip f-north)
        east (rotate north)
        f-east (h-flip north)
        v-east (v-flip f-north)
        south (rotate east)
        f-south (h-flip east)
        v-south (v-flip f-east)
        west (rotate south)
        f-west (h-flip south)
        v-west (v-flip f-south)
        xtra (rotate west)
        f-xtra (h-flip west)
        v-xtra (v-flip f-west)]
    (into #{} [north
               f-north
               v-north
               east
               f-east
               v-east
               south
               f-south
               v-south
               west
               f-west
               v-west
               xtra
               f-xtra
               v-xtra])))

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

(defn orient-tile [tiles the-grid x y]
  (when (and (pos? x) (pos? y) (< x (count the-grid)) (< y (count the-grid)))))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (mapv parse-tile (str/split input #"\n\n")))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (let [the-grid (find-neighbours input)]
    (println (Math/sqrt (count the-grid)))
    (println (map #(count (:neighbours %)) the-grid))
    (println (map #(count (:rotations %)) the-grid))
    (pp/pprint (:neighbours (first the-grid)))
    (println)
    (pp/pprint (:neighbours (first (filter #(= 2 (count (:neighbours %))) the-grid))))
    (apply * (map :id (filter #(= 2 (count (:neighbours %))) the-grid)))))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input])

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
