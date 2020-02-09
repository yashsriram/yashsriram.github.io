def title_and_lower_cases(word):
    lower_case = word.lower()
    title_case = lower_case[0].upper() + lower_case[1:]
    return title_case, lower_case


def case_combinations_recr(original_words_list, combinations, index, combination):
    if len(original_words_list) == index:
        combinations.append(combination)
        return
    title_case, lower_case = title_and_lower_cases(original_words_list[index])
    case_combinations_recr(original_words_list, combinations, index + 1, combination + [title_case])
    case_combinations_recr(original_words_list, combinations, index + 1, combination + [lower_case])


def case_combinations(token):
    # split() returns list with no empty strings, split(' ') can return list with empty strings
    words = token.split()
    lowered_words = [word.lower() for word in words]
    combinations = []
    case_combinations_recr(lowered_words, combinations, 0, [])
    joined_combinations = []
    for combination in combinations:
        joined_combinations.append(' '.join(combination))
    return joined_combinations


def heat_map(_min, val, _max):
    assert _min < _max
    gray = int(((val - _min) / (_max - _min)) * 255)
    if gray < 0:
        gray = 0
    if gray > 255:
        gray = 255
    return ('%02x' % gray).capitalize()
