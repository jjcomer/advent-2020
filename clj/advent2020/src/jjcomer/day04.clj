(ns jjcomer.day04
  (:require [clojure.string :as str]
            [jjcomer.util :as util]))

(def simple-check
  [#"byr:" #"iyr:" #"eyr:" #"hgt:" #"hcl:" #"ecl:" #"pid:"])

(defn do-simple-check [passport]
  (every? #(re-find % passport) simple-check))

(defn parse-input [input]
  (map #(str/replace % "\n" " ") (str/split input #"\n\n")))

(defn solve-part-1 []
  (let [passports (parse-input (util/get-input 4 :lines false))]
    (count (filter do-simple-check passports))))

(defn complex-test [passport]
  (and (re-find #"hcl:#[0-9a-f]{6}\b" passport)
       (re-find #"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b" passport)
       (re-find #"pid:\d{9}\b" passport)
       (if-let [[_ year] (re-find #"byr:(\d{4})\b" passport)]
         (<= 1920 (Integer. year) 2002))
       (if-let [[_ year] (re-find #"iyr:(\d{4})\b" passport)]
         (<= 2010 (Integer. year) 2020))
       (if-let [[_ year] (re-find #"eyr:(\d{4})\b" passport)]
         (<= 2020 (Integer. year) 2030))
       (if-let [[_ height unit] (re-find #"hgt:(\d{2,3})(cm|in)\b" passport)]
         (case unit
           "cm" (<= 150 (Integer. height) 193)
           "in" (<= 59 (Integer. height) 76)
           :else false))))

(defn solve-part-2 []
  (let [passports (parse-input (util/get-input 4 :lines false))]
    (count (filter complex-test passports))))
