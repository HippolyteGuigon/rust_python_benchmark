import time 
from multiprocessing import Pool, cpu_count

def compute(n: int)->int:
    s=0
    for i in range(n):
        s+=i*i
    return s

if __name__=="__main__":
    N=10000000
    WORKERS=cpu_count()

    t1=time.time()
    with Pool(WORKERS) as p:
        results=p.map(compute, [N]*WORKERS)

    elapsed=time.time()-t1

    print("Result: ", sum(results))
    print("Elapsed (python parrallel):", elapsed)