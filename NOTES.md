Search for all URLS of a certain pattern:

```
(?<!❍ )\b(?:https?://|www\.)(?![^ \n]*google\.com)\S+\b
```

```
(?<!❍ )\b(?:https?://|www\.)(?![^ \n]*(?:google\.com|gstatic\.com|googlesyndication\.com))\S+\b
```

```
(?<!❍ )\b(?:https?://|www\.)(?![^ \n"]*(?:google\.com|gstatic\.com|googlesyndication\.com))[^ \n"]+\b
```
