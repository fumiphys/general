# -*- coding: utf-8 -*-
import codecs
import datetime
import matplotlib.pyplot as plt
import sys


# plot time data
def plot_time(configfile):
    '''
    argument:
        configfile: string
    '''
    # variables
    config = {}
    config["plot_file"] = ""
    config["label"] = ""
    config["xlabel"] = ""
    config["ylabel"] = ""
    config["xcol"] = 1
    config["ycol"] = 2
    config["delimiter"] = "/"

    with codecs.open(configfile, 'r', 'utf-8') as reader:
        for line in reader:
            configline = line.strip().replace(" ", "").split("=")
            if configline[0] in config.keys() and not len(configline[1]) == 0:
                if configline[0] == "xcol" or configline[0] == "ycol":
                    config[str(configline[0])] = int(configline[1])
                else:
                    config[str(configline[0])] = str(configline[1])

    if not len(config["plot_file"]) == 0:
        xaxis = []
        yaxis = []
        with codecs.open(config["plot_file"], 'r', 'utf-8') as reader:
            for line in reader:
                values = line.split(",")
                if not (config["xcol"] < len(values) and config["xcol"] >= 0):
                    sys.exit()
                if not (config["ycol"] < len(values) and config["ycol"] >= 0):
                    sys.exit()
                xaxis.append(datetime.datetime.strptime(values[config["xcol"]], "%Y{delim}%m{delim}%d".format(delim=config["delimiter"])))
                yaxis.append(values[config["ycol"]])
        print(xaxis)
        plt.plot(xaxis, yaxis, label=config["label"])
        plt.xlabel(config["xlabel"])
        plt.ylabel(config["ylabel"])
        plt.xticks(xaxis, ["{0:%Y/%m/%d}".format(xdate) for xdate in xaxis], rotation='vertical', fontsize='small')
        plt.legend()
        plt.show()
# // plot time data end


# main
if __name__ == '__main__':
    plot_time("config.txt")
# // main
