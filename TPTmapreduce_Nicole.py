#TPT data:
#id, time, url, price(free if selected), grade (if selected), device(type of materials), typed search 

#time in format: 2016-02-29 02:59:41 UTC

from mrjob.job import MRJob
import numpy as np
import re

class SearchCount(MRJob):
    
    
    def mapper(self, _, line):
        data = line.split(',')
        
        #grab just the day portion of the date
        #can bin this however we want
        date = data[1][0:15]
     
        if 'Grade-Level' in line:
            l = line.find("Grade-Level")
            grade = (line[l:].split('/'))
            if len(grade) < 2:
                grade = 'none'
            else: 
                if ',' in grade[1]:
                    grade = grade[1].split(',')[0]
                else: 
                    grade = grade[1]

            key = date + ',' + grade
        else: key = date + ',none'
        
        key = key.lower()
        
        if 'page' not in key and 'order' not in key:
            yield key, 1
        
        
    def combiner(self, key, values):
        sum = 0
        
        for value in values:
            sum += value
            

        yield key, sum
        
        
        
    def reducer(self, key, values):
       
       sum = 0
       info = key.split(',')
       
       for value in values:
           sum += value
            
       yield info[0]+'0:00 UTC', (info[1],sum)
       
        
 
if __name__ == '__main__':
    SearchCount.run()
