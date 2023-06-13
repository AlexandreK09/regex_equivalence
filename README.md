# Regex Equivalence

Un projet que j'ai développé au cours de l'année après avoir étudier les expressions régulière.

Le but étant de déterminer si deux expressions régulières dénotent le même langage.

Après avoir étudier les expressions régulières, nous avons découvert dans un DM les congruences de Nerode permettant de déterminer l'automate minimal reconnaissant un langage. Je me suis très vite fait la réflexion que si deux automate reconnaissaient le même langage, alors l'automate minimale obtenu après application de l'algorithme de minimisation serait le même à une permutation des états près, ce qui nous donne une méthode pour déterminer si deux automates sont équivalents. Cependant, cette méthode ne me satisfait pas complétement car il reste assez difficile de déterminer si deux automates sont égaux à une permutations des états près.

J'ai donc continuer à réfléchir à d'autres façons de déterminer si deux expressions régulières sont équivalentes, quand un jour la solution m'est apparu comme une évidence. 
Les automates produits nous permettent de trouver un automate reconnaissant l'intersection de deux langage, de plus on sait aussi trouver l'automate reconnaissant le complémentaire d'un langage.
En prenant l'intersection d'un langage avec le complémentaire d'un autre langage, alors on obtient le langage de tous les mots du premier langages qui ne sont pas dans le second. Si ce langage est vide, alors le premier langage est inclus dans le second.
En procédant par double inclusion, on obtient une méthode permettant de déterminer si deux langages sont égaux.

Ce dépot contient mon implémentation de cette méthode.
