(ns fuel-calculator.core
  (:gen-class))

; Part 1 calculator
(defn fuel_calculator [module_mass]
  (- (int (Math/floor (/ module_mass 3))) 2))

; Part 1 verfy that the function performs as expected
(assert (= 2 (fuel_calculator 12)) "Fuel calculator is wrong!")
(assert (= 2 (fuel_calculator 14)) "Fuel calculator is wrong!")
(assert (= 654 (fuel_calculator 1969)) "Fuel calculator is wrong!")
(assert (= 33583 (fuel_calculator 100756)) "Fuel calculator is wrong!")

; Part 2 calculator
(defn recursive_fuel_calculator [initial_module_mass]
  ((fn [total, mass]
     (let [fuel_requirement (fuel_calculator mass)]
       (if (<= fuel_requirement 0 ) total (recur (+ total fuel_requirement) fuel_requirement)))
     ) 0 initial_module_mass)
  )

; Part 2 verify that the function performs as expected
(assert (= 966 (recursive_fuel_calculator 1969)) "Recursive fuel calculator is wrong!")
(assert (= 50346 (recursive_fuel_calculator 100756)) "Recursive fuel calculator is wrong!")

(defn convert_seq_of_strings_to_ints [ss]
  (map #(Integer/parseInt %) ss))

(defn get_fuel_list_from_module_mass_list [fuel_calc_func module_mass_list]
  (map fuel_calc_func (convert_seq_of_strings_to_ints module_mass_list)))

(defn get_total_fuel_requirement [fuel_calc_func module_mass_list]
  (reduce + (get_fuel_list_from_module_mass_list fuel_calc_func module_mass_list)))

(defn -main [& args]
  ; read from stdin to variable
  (let [module_mass_list (clojure.string/split-lines (slurp *in*))]
    (println "Part 1 result: " (get_total_fuel_requirement fuel_calculator module_mass_list))
    (println "Part 2 result: " (get_total_fuel_requirement recursive_fuel_calculator module_mass_list))
    )
  )

