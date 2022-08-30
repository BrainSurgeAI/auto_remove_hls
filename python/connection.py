import psycopg2
from config import config
from stmt import read_file


def connect(s):
    """ Connect to the PostgreSQL database server """
    conn = None
    try:
        # read connection parameters
        params = config()

        # connect to the PostgreSQL server
        print('Connecting to the PostgreSQL database...')
        conn = psycopg2.connect(**params)
		
        # create a cursor
        cur = conn.cursor()
        cur.execute(s)
        conn.commit()       
	# close the communication with the PostgreSQL
        cur.close()
    except (Exception, psycopg2.DatabaseError) as error:
        print(error)
    finally:
        if conn is not None:
            conn.close()
            print('Database connection closed.')


if __name__ == '__main__':
    # stms = read_file("../filelist.txt")
    # for s in stms:
    #     connect(s)
    removed_dir = "/Users/david/Desktop/encrypt_hls"
    stmts = read_file(removed_dir)
    for s in stmts:
        connect(s)
