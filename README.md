# web3-search

An exploration into finding the creation block of a web3 contract via weighted binary search. This is a useful approach when you have a correlation or where a loose heuristic can be applied to approximate the target value. In this case we often knew the approximate age of a web3 contract based on indexed transactions.

For this use case traditional binary search averages to around 27 iterations. This weighted approach greatly depends on the accuracy of the guess, however often based on sample data would converge in 12-14.

Our use case was to minimize the cost, however future work will go into making this parallel to reduce search time.
