subshader "jud_upplat_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	materialDiffuse 0.980392 0.592157 0.592157;
	texture "texture/buildings/judicator/jud_floor";
}

subshader "jud_upplat_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_floornospec";
}

subshader "jud_upplat_Material2" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_doorinside";
}

subshader "jud_upplat_Material3" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	envmap true;
	lightingSpecular true;
	texture "texture/buildings/judicator/jud_rivits";
}

