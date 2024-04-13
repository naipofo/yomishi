SELECT expression, reading, glossary, rules,
tags, definition_tags,
dictionary, dictionary.title as dictionary_name,
(SELECT id, category, dictionary, notes FROM $parent.tags WHERE dictionary not in $disabled_dicts) as tags,
(SELECT id, category, dictionary, notes FROM $parent.definition_tags WHERE dictionary not in $disabled_dicts) as definition_tags,
popularity, sequence
FROM term
WHERE expression in $terms and dictionary not in $disabled_dicts
ORDER BY popularity DESC, sequence DESC;
