/*
DE:
Rons Spell Helper
Schreibe eine App die es Ron erlaubt, in seinem Zauberspruchbuch:
 * Einen vorhandenen Eintrag anzuzeigen
 * Alle vorhandenen Zauberspruchnamen auszugeben
 * Einen neuen Zauberspruch einzutragen
 * Einen Zauberspruch zu entfernen
 * Nach dem das Programm die gewünschte Aufgabe erledigt hat, soll es erneut Fragen, welche Aktion ausgeführt werden
   soll, bis die Nutzer*in sich entscheidet das Programm zu verlassen

Erstelle hierzu:
- ein struct Spell mit den Feldern name, description
- ein struct Spellbook mit einem Vektor von Spells
- implementiere die Funktionen:
  - new: Ersellt eine neues Spellbook mit 5 Spells:
        Spell { name: "Expelliarmus".to_string(), description: "Entwaffnungszauber".to_string() },
        Spell { name: "Lumos".to_string(), description: "Beleuchtungszauber".to_string() },
        Spell { name: "Wingardium Leviosa".to_string(), description: "Schwebezauber".to_string() },
        Spell { name: "Accio".to_string(), description: "Aufrufezauber".to_string() },
        Spell { name: "Nox".to_string(), description: "Verdunklungszauber".to_string() },
  - add_spell: Fügt einen neuen Spell zum Spellbook hinzu
  - remove_spell: Entfernt einen Spell aus dem Spellbook
  - list_spells: Gibt alle Spellnamen aus dem Spellbook aus
  - find_spell: Gibt einen Spell aus dem Spellbook aus

EN:
Ron's Spell Helper
Write an app that allows Ron to display an existing entry in his spell book:
 * Display an existing entry
 * Display all existing spell names
 * Add a new spell
 * Remove a spell
 * After the program has completed the desired task, it should ask again which action is to be carried out until the user has completed it.
   until the user decides to exit the program

To do this, create
- a struct Spell with the fields name, description
- a struct Spellbook with a vector of spells
- implement the functions:
  - new: Creates a new spellbook with 5 spells:
        Spell { name: "Expelliarmus".to_string(), description: "Disarm Spell".to_string() },
        Spell { name: "Lumos".to_string(), description: "Illumination spell".to_string() },
        Spell { name: "Wingardium Leviosa".to_string(), description: "Levitation spell".to_string() },
        Spell { name: "Accio".to_string(), description: "Invocation spell".to_string() },
        Spell { name: "Nox".to_string(), description: "Darkening spell".to_string() },
  - add_spell: Adds a new spell to the spellbook
  - remove_spell: Removes a spell from the spellbook
  - list_spells: Returns all spell names from the spellbook
  - find_spell: Returns a spell from the spellbook

*/
