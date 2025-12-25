import time 

def compute_sum(n):
    s=0
    for i in range(n):
        s+=i**2
    return s

N=10000000
time1=time.time()
result=compute_sum(N)
time2=time.time()
print(f"Time: {time2-time1:.3f}","Result: ", result)