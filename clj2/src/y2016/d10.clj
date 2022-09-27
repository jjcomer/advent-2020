(ns y2016.d10
  (:require [clojure.test :as t :refer [deftest]]
            [clojure.string :as str]))

;; PROBLEM LINK https://adventofcode.com/2016/day/10

;; Generator Logic

(comment {:bot-id {:chips []
                   :rules {:low :bot-id
                           :high :bot-id}}})

(def intake-pattern #"^value (\d*) goes to bot (\d*)$")
(def passing-pattern #"^bot (\d*) gives low to (bot|output) (\d*) and high to (bot|output) (\d*)$")

(defn try-intake [line]
  (when-let [[_ value bot] (re-find intake-pattern line)]
    {:value (parse-long value) :bot (parse-long bot)}))

(defn try-passing [line]
  (when-let [[_ bot low_type low_target high_type high_target] (re-find passing-pattern line)]
    {:bot (parse-long bot)
     :low (case low_type
            "output" (keyword (str "output" low_target))
            (parse-long low_target))
     :high (case high_type
             "output" (keyword (str "output" high_target))
             (parse-long high_target))}))

(defn get-bot [bots bot]
  (get bots bot {:bot bot :chips #{} :rules {}}))

;; Solution Logic

(defn process-bots [bots]
  (loop [bots bots to-check (vals bots)]
    (if-let [{:keys [chips bot rules]} (first to-check)]
      (if (= 2 (count chips))
        (let [min-chip (apply min chips)
              max-chip (apply max chips)
              bots (cond-> bots
                     true (assoc-in [bot :chips] [])
                     (number? (:low rules)) (update-in [(:low rules) :chips] conj min-chip)
                     (keyword? (:low rules)) (update-in [(:low rules)] (fn [l c] (if l (conj l c) [c])) min-chip)
                     (number? (:high rules)) (update-in [(:high rules) :chips] conj max-chip)
                     (keyword? (:high rules)) (update-in [(:high rules)] (fn [l c] (if l (conj l c) [c])) max-chip))]
          (recur bots (rest to-check)))
        (recur bots (rest to-check)))
      bots)))

;; Entry Points

(defn generator
  "The generator fn is used to parse your input into. The output of this fn will be passed into each of the solving fns"
  [input]
  (reduce (fn [acc line]
            (if-let [{:keys [value bot]} (try-intake line)]
              (let [current-bot (get-bot acc bot)]
                (assoc acc bot (update current-bot :chips conj value)))
              (if-let [{:keys [bot low high]} (try-passing line)]
                (let [current-bot (get-bot acc bot)]
                  (assoc acc bot (assoc current-bot :rules {:low low :high high})))
                (throw (ex-info "Error parsing" {:data line})))))
          {}
          (str/split-lines input)))

(defn solve-part-1
  "The solution to part 1. Will be called with the result of the generator"
  [input]
  (let [target #{61 17}]
    (loop [bots input]
      (let [new-bots (process-bots bots)]
        (if-let [answer-bot (first (filter #(= target (:chips %)) (vals new-bots)))]
          (:bot answer-bot)
          (recur new-bots))))))

(defn solve-part-2
  "The solution to part 2. Will be called with the result of the generator"
  [input]
  (loop [bots input]
    (let [new-bots (process-bots bots)]
      (if (= new-bots bots)
        (apply * (mapcat identity (vals (select-keys new-bots [:output1 :output0 :output2]))))
        (recur new-bots)))))

;; Tests
;; Use tests to verify your solution. Consider using the sample data provided in the question

(deftest sample-test
  (t/is (= 2 (+ 1 1))))
