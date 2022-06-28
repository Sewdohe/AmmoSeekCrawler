# AmmoSeekCrawler
Crawls brass ammunition prices on ammoseek.com...well, it's supposed to.

For some reason, in the for loop of all the table rows the find_element function always returns the value of the first row, even though it should only return children of the current element. I'm sure this has to do with the nuances of Rust that I'm unaware of...
