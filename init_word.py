# init_word.py

query = input("Please input the query you want to look at: ")

f = open("single_query.txt", "w")
f.write(query)
f.close()
