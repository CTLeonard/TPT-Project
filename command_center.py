# command center to run hadoop and other programs
from subprocess import call
import re
import sys

def CodeZero():
    # Begin by defining a query.
    call(["python", "init_word.py"])


def CodeOne(path):
    # Open a file for console to output to
    f = open("word_result2.csv", "w+")

    # call map reduce
    call(["python", "singleFrequency.py", path], stdout=f)
    f.close()

def CodeTwo():
    # Messy. regexp's to format text
    f2 = open("word_result2.csv", "r+")
    f3 = open("word_result3.csv", "w+")
    #print("First call done")
    for line in f2:
        # removes tab, null, and quotation marks
        line3 = re.sub(r'\t|\"|null', '', line)
        print(line3)
        f3.write(line3)
    f2.close()
    f3.close()

def CodeThree():
    # Knit Rmd file that reads in word_result3.csv and plots a single time series. Then opens the knitted pdf.
    # needed to use Sys.getenv("RSTUDIO_PANDOC") to get where pandoc is located
    call(["Rscript", "-e", "library(rmarkdown);Sys.setenv(RSTUDIO_PANDOC='C:/Program Files/RStudio/bin/pandoc');rmarkdown::render('plot_frequency.Rmd')"], shell=True)
    call(["plot_frequency.pdf"], shell=True)

    
if __name__ == "__main__":
    path_to_data = sys.argv[1]

    CodeZero()
    CodeOne(path_to_data)
    CodeTwo()
    CodeThree()

'''
Notes

Run python command wrapper with arg being path to data to search over for query. My data has first row removed.
  python .\command_center.py ..\Group_1\small_tpt.csv 
  Or for larger data
  python .\command_center.py ..\Group_1\tpt_data.csv

When asked, input the query you want to generate a time series for.

Dependencies
Using python 3.6.1, powershell (administrator mode for mrjob to run)
Need myjob installed. Thanks to python 3.6, can us pip to install
  pip install mrjob
Need fuzzywuzzy installed
  pip install fuzzywuzzy
Needed to add Rscript (after adding library) to path.
  $env:Path += ";C:\Program Files\R\R-3.3.1\bin"
Pandoc location in CodeThree might be different.
Setting correct path in plot_frequency.Rmd

Running on the large dataset takes about five minutes.
'''
