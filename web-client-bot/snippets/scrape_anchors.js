Array.from(document.querySelectorAll('a'))
    .filter(a => a.href && !a.href.startsWith('javascript:') && a.offsetParent !== null)
    .map(a => ({
        href: typeof a.href === 'string' ? a.href : '',
        text: (a.textContent || '').trim()
    }))