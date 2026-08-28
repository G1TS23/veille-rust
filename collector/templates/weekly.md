{{! ─────────────────────────────────────────────────────────────────────── }}
{{! Corps de l'issue hebdomadaire. Éditable sans recompiler le collecteur.  }}
{{!                                                                        }}
{{! Valeurs : {{date}} {{count}} {{top_count}} {{rest_count}}              }}
{{!           {{items}} {{rest}}                                           }}
{{!                                                                        }}
{{! Blocs   : [[#has_items]]…[[/has_items]]  s'il y a des items            }}
{{!           [[#empty]]…[[/empty]]          si la semaine est vide        }}
{{!           [[#has_rest]]…[[/has_rest]]    s'il y a un reste à replier   }}
{{!                                                                        }}
{{! Lignes  : les régions [[#item]] et [[#rest_item]] en bas de fichier    }}
{{!           définissent le format d'UNE ligne. Placeholders dispo :      }}
{{!           {{title}} {{url}} {{source}} {{score}} {{tags}} {{summary}}  }}
{{!                                                                        }}
{{! Ces lignes de commentaire ne sont jamais rendues.                      }}
{{! ─────────────────────────────────────────────────────────────────────── }}
Récap de la semaine écoulée, arrêté au {{date}}.

[[#has_items]]
**{{count}} items** collectés. Top {{top_count}} à trier :

{{items}}
[[/has_items]]
[[#empty]]
Rien de neuf cette semaine.
[[/empty]]
[[#has_rest]]
<details><summary>Les {{rest_count}} autres</summary>

{{rest}}

</details>
[[/has_rest]]

---
_Généré automatiquement. Coche au fil de tes lectures, ferme l'issue quand tu as fait le tri._

[[#item]]- [ ] [{{title}}]({{url}}) — `{{source}}` · score {{score}}[[/item]]
[[#rest_item]]- [{{title}}]({{url}}) — `{{source}}`[[/rest_item]]
