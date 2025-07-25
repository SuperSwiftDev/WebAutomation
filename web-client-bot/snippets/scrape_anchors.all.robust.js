Array.from(document.querySelectorAll('a'))
    .map(a => {
        const href = typeof a.href === 'object' && a.href.baseVal ? a.href.baseVal : a.href;
        return {
            href,
            text: (a.textContent || '').trim()
        };
    })
    .filter(link => typeof link.href === 'string' && link.href && !link.href.startsWith('javascript:') && link.text && link.text.length > 0)