SELECT expression, reading, glossary, rules,
tags, definition_tags,
dictionary, dictionary.title as dictionary_name,
(SELECT id, category, dictionary, notes FROM $parent.tags) as tags,
(SELECT id, category, dictionary, notes FROM $parent.definition_tags) as definition_tags,
popularity, sequence
FROM term
WHERE expression in $terms
ORDER BY popularity DESC, sequence DESC;
