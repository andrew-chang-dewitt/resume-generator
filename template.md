# {header.name}


${ header.contactMethod.forEach(m => `
} }
`) }
- ${ match m {
    ContactMethod::Phone => "[tel:+{}-{}]"
    ContactMethod::Email
    ContactMethod::Link
