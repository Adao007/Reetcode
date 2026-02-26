import csv

def make_salesmen_dictionary():
    result = {}  # Dictionary

    with open('employees.csv', newline='') as f:
        reader = csv.DictReader(f)
        for row in reader:
            result[row['id']] = {
                'name': row['name'],
                'location': row['location'], 
                'manager_id': row['manager_id']
            }

    for key, dictionary in result.items():
        manager_id = dictionary['manager_id']
        if manager_id and manager_id in result:
            dictionary['manager_id'] = result[manager_id]['name']
        else: 
            dictionary['manager_id'] = None
        
    return result

def add_annual_goal(result):
    with open('annual_goals.csv', newline='') as f: 
        reader = csv.reader(f, delimit=',')
        for row in reader:
            if row[1] in result: 

def main(): 
    test = make_salesmen_dictionary()
    print(test)


if __name__ == '__main__':
    main()