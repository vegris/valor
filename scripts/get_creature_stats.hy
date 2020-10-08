(import [bs4 [BeautifulSoup]])

(setv PAGE "creatures.html")
(setv SELECTOR ".sortable > tbody:nth-child(2) > tr")

(setv ROWS (.split "name town level attack defence damage_min damage_max health speed"))

(defn update-key [dictionary key update-func]
  (->> key 
    (get dictionary)
    (update-func)
    (assoc dictionary key))
  dictionary)

(defn get-span-value [element]
  (-> element
    (.find "span")
    (. string)
    (.strip)))

(defn parse-row [row]
  (as-> row it
    (.select it "td")
    (zip ROWS it)
    (dict it)
    (update-key it "name" 
      (fn [v] 
        (-> v
          (.find "a")
          (get "title"))))
    (update-key it "town" 
      (fn [v]
        (-> v
          (.find "span")
          (get "title"))))
    (update-key it "level"
      (fn [v]
        (setv span-conts
          (-> v 
            (.find "span")
            (. contents)))
        (setv lvl (-> span-conts (get 0) (.strip)))
        (if
          (> (len span-conts) 1)
          (+ lvl "+")
          lvl)))
    (reduce
      (fn [d key] (update-key d key get-span-value))
      (cut ROWS 3)
      it)))

(with [f (open PAGE)]
  (as-> f it
    (.read it)
    (BeautifulSoup it "html.parser")
    (.select it SELECTOR)
    (map parse-row it)
    (map 
      (fn [creature]
        (setv name (-> creature (get "name") (.replace " " "")))
        (setv attack (get creature "attack"))
        (setv defence (get creature "defence"))
        (setv damage_min (get creature "damage_min"))
        (setv damage_max (get creature "damage_max"))
        (setv health (get creature "health"))
        (setv speed (get creature "speed"))

        (print f"Self::{name} => CreatureStats" "{")
        (print f"\tattack: {attack},")
        (print f"\tdefence: {defence},")
        (print f"\tdamage: ({damage_min}, {damage_max}),")
        (print f"\thealth: {health},")
        (print f"\tspeed: {speed}")
        (print "},")
      )
      it)
    (list it)))
    
