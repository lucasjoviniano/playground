def levenshtein_distance_private(one, two, len1, len2):

	if len1 == 0:
		return len2

	if len2 == 0:
		return len1

	cost = 0

	# testa o último caractere
	if one[len1 - 1] != two[len2 - 1]:
		cost = 1

	return min(levenshtein_distance_private(one, two, len1 - 1, len2) + 1,
				levenshtein_distance_private(one, two, len1, len2 - 1) + 1,
				levenshtein_distance_private(one, two, len1 - 1, len2 - 1) + cost)

def levenshtein_distance(one, two):
    return levenshtein_distance_private(one, two, len(one), len(two))
    
one, two = 'Hello', 'Jello'
print(levenshtein_distance(one, two))