# Random city name generator
This generator allows you to generate random city names from the specified country.
Implementation using Markov Chains allows the generation of names that do not necessarily exist but sound real.
Works properly with UTF-8 encoded characters.

It is possible to generate names from four countries:
  - Poland
  - Japan
  - Ukraine
  - United Kingdom
  
There are four countries for now because (as you can see in the repo) I provided four files with city names. These files are used for 'training' Markov Chains. 

# Usage

Install Rust

Clone the project

'''
git clone https://github.com/bartoszpanek1/City-Names-Generator.git
'''

Run the program using command line

'''
cargo run -- <country_code> <num_cities>
'''
num_cities - how many cities to generate

Possible country codes:
  - pl
  - jp
  - ua
  - uk
  
# Examples

Generate 5 random Polish-like cities
'''
cargo run -- pl 5
zewielkatów
golub-dole
gowskowa-zd
częstartuzynański
mrągogrzyny
'''

Generate 6 random Japanese-like cities
'''
cargo run -- jp 6
fujisa
fukure
kasu
numazuokosek
osakashinomi
okyona
'''

Generate 3 random Ukraine-like cities
'''
cargo run -- ua 3
uzyn
pavlohirsk
znamianske
'''

Generate 2 random UK-like cities
'''
cargo run -- uk 2
east parlesteig
ellshirle
'''

It works pretty good. However, UK and Poland is visibly worse than Japan and Ukraine. There is still space for improvement.
