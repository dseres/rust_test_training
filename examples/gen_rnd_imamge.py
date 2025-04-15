import random

if __name__ == "__main__":
    for i in range(0,50):
        for j in range(0,50):
            print(random.randint(0 ,  255), end="")
            if j < 50 :
                print(" ", end="")
        print()