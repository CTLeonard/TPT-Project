# singleFrequency.py
# import modules and data
from mrjob.job import MRJob
from mrjob.step import MRStep
from fuzzywuzzy import fuzz

class MrWord(MRJob):        

    f = open("single_query.txt", "r")
    query = f.readline()
    f.close()
    
    def mapper(self, _, line):
        s2 = line.split(',')
              
        long_time = s2[1]
        time = long_time.split(' ')[0]
        search = s2[6]
        closeness = 0
        
        # some queries were empty, or had unusual characters like #, !, '
        if search and search[0].isalpha():
            closeness = fuzz.token_set_ratio(search, self.query)

        if closeness > 75:
            yield time, search
        

    def reducer_init(self):
        self.inner_senshi = {}
        self.inner_start = 0
        
    def reducer(self, key, value):
        
        for v in value:
            if self.inner_start == 0:
                self.inner_senshi[v] = 0
                self.inner_start = 1

            if v in self.inner_senshi.keys():
                self.inner_senshi[v] = self.inner_senshi[v] + 1
            else:
                self.inner_senshi[v] = 1
            
        for magic_key, magic_val in self.inner_senshi.items():
            yield key + "," + magic_key+","+str(magic_val), None

            
        
if __name__ == '__main__':
    MrWord.run()
