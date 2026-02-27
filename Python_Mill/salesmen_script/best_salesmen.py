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
        reader = csv.reader(f, delimiter=',')
        for row in reader:
            if row[0] in result: 
                result[row[0]]['annual_goal'] = int(row[2]) * 4
    
    return result

def add_percentage_of_sales(result):
    list = []
    with open('sales_record.csv', newline='') as f:
        reader = csv.reader(f, delimiter=',')
        for row in reader: 
            list.append(row)

    list = modify_list(list)

    for entry in list: 
        if entry[4] == 'closed':
            key = entry[0]
            if key in result:
                if 'percentage' in result[key]:
                    result[key]['percentage'] += int(entry[3])
                else:
                    result[key]['percentage'] = int(entry[3])

    for i in result:
        if 'percentage' in result[i]:
            result[i]['percentage'] = int(result[i]['percentage']) / int(result[i]['annual_goal'])

    return result

def modify_list(list):
    for entry in list: 
        match entry[2]:
            case 'Jupi':
                entry[0] = '1'
            case 'Gabe':
                entry[0] = '3'
            case 'Tony':
                entry[0] = '4'
            case 'Azalea':
                entry[0] = '7'
            case 'Clara':
                entry[0] = '8'
            case _: 
                print("Not valid entry/Not sales employee!")

    return list

def get_best_salesmen(result):
    list = []
    for entry in result:
        if 'percentage' in result[entry]:
            list.append((result[entry]['percentage'], entry))

    sorted_list = sorted(list)
    return sorted_list



def main(): 
    test = make_salesmen_dictionary()
    test = add_annual_goal(test)
    test = add_percentage_of_sales(test)
    order = get_best_salesmen(test)
    
    for position in reversed(order): 
        print(test[(position[1])])


if __name__ == '__main__':
    main()