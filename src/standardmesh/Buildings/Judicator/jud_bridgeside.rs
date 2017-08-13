subshader "jud_bridgeside_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_bridgeside_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	lightingSpecular true;
	twosided true;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_bridgeside_Material2" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	lightingSpecular true;
	twosided true;
	texture "texture/buildings/judicator/jud_frontpanel";
}

subshader "jud_bridgeside_Material3" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	lightingSpecular true;
	twosided true;
	texture "texture/buildings/judicator/jud_pipe";
}

subshader "jud_bridgeside_Material4" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	materialSpecular 0.1 0.1 0.1;
	envmap true;
	lightingSpecular true;
	twosided true;
	texture "texture/buildings/judicator/jud_wall01";
}

subshader "jud_bridgeside_Material5" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	materialSpecular 0.5 0.5 0.5;
	envmap false;
	lightingSpecular false;
	transparent true;
      	texture "texture/buildings/judicator/jud_glass";
}
