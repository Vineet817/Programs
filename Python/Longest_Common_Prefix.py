def LCP(strs):
    if not strs:
        return ""
    shortstr = min(strs, key=len)
    com=""
    for i in range(len(shortstr)):
        for string in strs:
           if  string[i]!=shortstr[i]:
               return com
        com+=shortstr[i]

    return com

ar=["iso","isokd","in"]

print(LCP(ar))