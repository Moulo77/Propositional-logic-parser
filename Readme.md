# Analyseur logique

Cet outil vous permet d'analyser des formules logiques et de vérifier leur satisfiabilité en calculant la table de verite.

## Installation

Assurez-vous d'avoir installé Rust et Cargo. Si ce n'est pas déjà fait, vous pouvez les installer à partir du site officiel de Rust : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Accédez au répertoire du projet :

```bash
cd tp_logique
```

Compilez le projet avec Cargo :

```bash
cargo build --release
```

L'exécutable sera généré dans le répertoire `target/release/TP_logique`, ou le lancer avec 
```bash
cargo run --release
```

## Utilisation

Pour utiliser l'analyseur logique, suivez les étapes suivantes :

1. Rédigez une formule logique en utilisant les opérateurs logiques `and`, `or`, `not`, `if`, `iff`, `then`, ainsi que des propositions composees de lettres uniquement.
2. Exécutez l'analyseur logique en fournissant votre formule en tant qu'entrée.

Voici un exemple d'utilisation :

```bash
./target/release/analyseur-logique
```

Vous serez invité à entrer une formule logique. Entrez votre formule et appuyez sur Entrée. L'analyseur logique générera alors toutes les combinaisons possibles de valeurs de vérité et verifira la satisfiabilité de la formule.

## Exemples

- Formule : `ilpleut and fenetreouverte`
  - Résultats :
    - Assignations satisfaisables :
      - `{"ilpleut": true, "fenetreouverte": true`
    - Assignations insatisfaisables :
      - `{"ilpleut": false, "fenetreouverte": false}`
      - `{"ilpleut": true, "fenetreouverte": false}`
      - `{"ilpleut": false, "fenetreouverte": true}`
